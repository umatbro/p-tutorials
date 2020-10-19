terraform {
    source = "../../../modules/lambda"
}

include {
    path = find_in_parent_folders()
}
