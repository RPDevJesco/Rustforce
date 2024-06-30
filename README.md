# RustForce

This is a basic implementation example of how to connect to Salesforce from a Rust application.

In order to use this, you need to update the auth_url to be set to the correct URL for your Salesforce instance.

This application currently only allows for you to authorize a connection to the salesforce org, insert a record and query for records in Salesforce.
First run of the application will generate the salesforce_config.ini file, you will need to fill in the details.
SalesforceVersionNumber
CONSUMER_KEY
CONSUMER_SECRET
USERNAME
PASSWORD
TOKEN
ENDPOINT