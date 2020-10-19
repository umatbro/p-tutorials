terraform {
    source = "../../../modules/ec2"
}

include {
    path = find_in_parent_folders()
}

inputs = {
    main_instance_type = "t2.nano"
}
