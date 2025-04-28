#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, Address, symbol_short, String};

// Donation data structure
#[contracttype]
#[derive(Clone)]
pub struct Donation {
    pub donation_id: u64,
    pub donor: Address,
    pub campaign: u64,
    pub amount: u64,
    pub message: String,
    pub timestamp: u64,
}

// Campaign data structure
#[contracttype]
#[derive(Clone)]
pub struct Campaign {
    pub campaign_id: u64,
    pub organizer: Address,
    pub title: String,
    pub description: String,
    pub goal_amount: u64,
    pub current_amount: u64,
    pub donor_count: u64,
    pub active: bool,
    pub creation_time: u64,
}

// Contract storage keys
const DONATION_COUNT: Symbol = symbol_short!("DON_COUNT");
const CAMPAIGN_COUNT: Symbol = symbol_short!("CAM_COUNT");

// Mapping donation ID to donation data
#[contracttype]
pub enum DonationMap {
    Donation(u64)
}

// Mapping campaign ID to campaign data
#[contracttype]
pub enum CampaignMap {
    Campaign(u64)
}

#[contract]
pub struct DonationTrackerContract;

#[contractimpl]
impl DonationTrackerContract {
    // Create a new fundraising campaign
    pub fn create_campaign(env: Env, organizer: Address, title: String, description: String, goal_amount: u64) -> u64 {
        // Get current campaign count
        let mut campaign_count = env.storage().instance().get(&CAMPAIGN_COUNT).unwrap_or(0);
        campaign_count += 1;
        
        // Create campaign record
        let campaign = Campaign {
            campaign_id: campaign_count,
            organizer: organizer,
            title: title,
            description: description,
            goal_amount: goal_amount,
            current_amount: 0,
            donor_count: 0,
            active: true,
            creation_time: env.ledger().timestamp(),
        };
        
        // Store the campaign record
        env.storage().instance().set(&CampaignMap::Campaign(campaign_count), &campaign);
        env.storage().instance().set(&CAMPAIGN_COUNT, &campaign_count);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Campaign {} created with goal amount {}", campaign_count, goal_amount);
        
        campaign_count
    }
    
    // Make a donation to a campaign
    pub fn donate(env: Env, donor: Address, campaign_id: u64, amount: u64, message: String) -> u64 {
        // Get campaign record
        let mut campaign: Campaign = env.storage().instance().get(&CampaignMap::Campaign(campaign_id))
            .unwrap_or_else(|| panic!("Campaign not found"));
        
        // Check if campaign is active
        if !campaign.active {
            log!(&env, "Campaign is not active");
            panic!("Campaign is not active");
        }
        
        // Get current donation count
        let mut donation_count = env.storage().instance().get(&DONATION_COUNT).unwrap_or(0);
        donation_count += 1;
        
        // Create donation record
        let donation = Donation {
            donation_id: donation_count,
            donor: donor,
            campaign: campaign_id,
            amount: amount,
            message: message,
            timestamp: env.ledger().timestamp(),
        };
        
        // Store the donation record
        env.storage().instance().set(&DonationMap::Donation(donation_count), &donation);
        env.storage().instance().set(&DONATION_COUNT, &donation_count);
        
        // Update campaign statistics
        campaign.current_amount += amount;
        campaign.donor_count += 1;
        env.storage().instance().set(&CampaignMap::Campaign(campaign_id), &campaign);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Donation {} made to campaign {} with amount {}", donation_count, campaign_id, amount);
        
        donation_count
    }
    
    // Get campaign details
    pub fn get_campaign(env: Env, campaign_id: u64) -> Campaign {
        env.storage().instance().get(&CampaignMap::Campaign(campaign_id))
            .unwrap_or_else(|| panic!("Campaign not found"))
    }
    
    // Get donation details
    pub fn get_donation(env: Env, donation_id: u64) -> Donation {
        env.storage().instance().get(&DonationMap::Donation(donation_id))
            .unwrap_or_else(|| panic!("Donation not found"))
    }
}