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
    
    // Test getting incidents by ID - now returns Incident directly, not Option
    let retrieved1 = client.get_incident(&1);
    let retrieved2 = client.get_incident(&2);
    
    // Check that valid incidents are retrieved correctly
    assert_eq!(retrieved1.id, 1);
    assert_eq!(retrieved1.report, String::from_str(&env, "Server down"));
    assert_eq!(retrieved2.id, 2);
    assert_eq!(retrieved2.report, String::from_str(&env, "Database error"));
    
    // Test total incidents count
    assert_eq!(client.get_total_incidents(), 2);
    
    // Test non-existent incident - now returns default incident with id = 0
    let non_existent = client.get_incident(&999);
    assert_eq!(non_existent.id, 0); // id = 0 indicates "not found"
    assert_eq!(non_existent.report, String::from_str(&env, "")); // empty report for not found
}