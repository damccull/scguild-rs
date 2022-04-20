# Events
Data can be stored as events. A "rehydrator" repo will be used to query the data and instantiate one or more aggregates, depending on the query results, then pass the remaining events to the aggregate to fold into itself.


## Manufacturers
### Events
* Manufacturer Added
* Manufacturer Set
    * Allow partial field changes if possible, otherwise make an event for each field.
* Manufacturer Removed

### Common Fields
* AggregateId
* AggregateVersion
* Timestamp


## Ship Models

### Events
* Ship Model Added
* Ship Model Set
    * Allow partial field changes if possible, otherwise make an event for each field.
* Ship Model Removed
* Ship Component Added
* Ship Component Set
* Ship Component Removed

### Common Fields
* AggregateId
* AggregateVersion
* Timestamp


## User Ships
### Events
* User Ship Commissioned
* User Ship Set
* User Ship Decommissioned

### Common Fields
* AggregateId
* AggregateVersion
* Timestamp

## User Accounts

_NOTE: No authentication data (e.g. passwords/tokens). Store those in a regular table._