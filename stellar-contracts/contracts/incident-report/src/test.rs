#![cfg(test)]
use crate::{IncidentReportContract, IncidentReportContractClient};
use soroban_sdk::{Env, String};

#[test]
fn test_report_and_get_incident() {
    let env = Env::default();
    let contract_id = env.register(IncidentReportContract, ());
    let client = IncidentReportContractClient::new(&env, &contract_id);

    // Report first incident
    let report1 = String::from_str(&env, "Server down");
    let incident1 = client.report_incident(&report1);
    
    // Report second incident
    let report2 = String::from_str(&env, "Database error");
    let incident2 = client.report_incident(&report2);
    
    // Test incident IDs increment correctly
    assert_eq!(incident1.id, 1);
    assert_eq!(incident2.id, 2);
    
    // Test getting incidents by ID
    let retrieved1 = client.get_incident(&1);
    let retrieved2 = client.get_incident(&2);
    
    assert!(retrieved1.is_some());
    assert!(retrieved2.is_some());
    
    // Test total incidents count
    assert_eq!(client.get_total_incidents(), 2);
    
    // Test non-existent incident
    let non_existent = client.get_incident(&999);
    assert!(non_existent.is_none());
}