{
    "variables": {
    },
    "targets": [
        {
            "target_name": "node-caseconv",
            "sources": [ "<!@(find src -type f -name '*.cpp')" ],
            "include_dirs": [
                "../../include"
            ],
            "link_settings": {
                "libraries": [ "../../../target/release/libcaseconv.a" ]
            }
        }
    ]
}
