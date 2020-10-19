data "aws_ami" "amazon_linux" {
    most_recent = true
    owners = ["amazon"]
    
    filter {
        name   = "owner-alias"
        values = ["amazon"]
    }

    filter {
        name = "name"
        values = ["amzn2-ami-hvm*"]
    }
}

resource "aws_instance" "main" {
    ami = data.aws_ami.amazon_linux.id
    instance_type = var.main_instance_type

    tags = {
        Name = "${var.env}-main-instance"
    }
}
