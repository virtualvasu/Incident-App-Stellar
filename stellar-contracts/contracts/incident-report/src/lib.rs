#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, String, Address, Map};

const COUNTER: Symbol = symbol_short!("COUNTER");
const INCIDENTS: Symbol = symbol_short!("INCIDENTS");

#[derive(Clone)]
#[contracttype]
pub struct Incident {
    pub id: u32,
    pub report: String,
    pub timestamp: u64,
    pub reporter: Address,
}

#[contract]
pub struct IncidentReportContract;

#[contractimpl]
impl IncidentReportContract {
    /// Report an incident and return the incident details
    pub fn report_incident(env: Env, report: String) -> Incident {
        // Get the caller's address
        let reporter = env.current_contract_address();
        
        // Get current timestamp
        let timestamp = env.ledger().timestamp();
        
        // Get and increment the counter
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count += 1;
        
        // Create the incident
        let incident = Incident {
            id: count,
            report: report.clone(),
            timestamp,
            reporter: reporter.clone(),
        };
        
        // Get existing incidents map or create new one
        let mut incidents: Map<u32, Incident> = env
            .storage()
            .instance()
            .get(&INCIDENTS)
            .unwrap_or_else(|| Map::new(&env));
        
        // Store the incident
        incidents.set(count, incident.clone());
        
        // Update storage
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().set(&INCIDENTS, &incidents);
        
        // Extend TTL for data persistence
        env.storage().instance().extend_ttl(50, 100);
        
        incident
    }
    
    /// Get incident details by ID
    pub fn get_incident(env: Env, incident_id: u32) -> Option<Incident> {
        let incidents: Map<u32, Incident> = env
            .storage()
            .instance()
            .get(&INCIDENTS)
            .unwrap_or_else(|| Map::new(&env));
        
        incidents.get(incident_id)
    }
    
    /// Get the total number of incidents reported
    pub fn get_total_incidents(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
}

mod test;