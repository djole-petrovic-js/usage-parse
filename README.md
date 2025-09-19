*Simple project in Rust to parse all usage log files and update user usages.*

The program should first parse all command line interface arguments.
After that, it parses every log file and assembles the aggregate usage struct.
Currently, nothing is being done with the aggregation, it is just printed. In a production scenario, it should update a database, call a webhook, or perform another action.

This project is a learning experience, so DO NOT use this code in production.