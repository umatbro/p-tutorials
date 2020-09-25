There will be 2 environments:
* Staging
* Production

Architecture:
* EC2 instance
* Dynamodb table
* Lambda function (stream handler)

Differences:
* Different EC2 instance on different environments
* Different read/write capacity units
* Lambda function is the same.


**TODO**
* Try to change DynamoDB's `billing_mode` across the environments. `PAY_PER_REQUEST` does not require write capacity units, `PROVISIONED` does - what if I want `PAY_PER_REQUEST` on staging and `PROVISIONED` on production?


## Notes from docs

Given the following structure:
```
└── modules
    ├── dynamodb
    │   └── main.tf
    ├── ec2
    │   └── main.tf
    └── lambda
        └── main.tf
```

This repo contains typical Terraform code, with one difference: anything in your code that should be different between environments should be exposed as an input variable.

Example:
```
variable "instance_type" {
  description = "What kind of servers to run (e.g. t2.large)"
}
```