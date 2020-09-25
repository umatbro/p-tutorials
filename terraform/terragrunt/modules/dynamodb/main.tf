resource "aws_dynamodb_table" "terragrunt-tut" {
    name = "${var.env}-terragrunt-tut"
    hash_key = "id"

    billing_mode = "PAY_PER_REQUEST"
    
    attribute {
        name = "id"
        type = "S"
    }
}
