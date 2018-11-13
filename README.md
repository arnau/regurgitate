# Regurgitate

## content layout

```
content
└── {organisation}
    └── {dataset}
        └── README.md
        └── latest.csv-metadata.json
        └── latest.csv
        └── snapshots
            └── {snapshot-checksum}.csv-metadata.json
            └── {snapshot-checksum}.csv
            └── …
        └── …
    └── …
```

## output layout

```
build
└── {organisation}
    └── {dataset}
        └── index.html
        └── latest.csv-metadata.json
        └── latest.csv
        └── snapshots
            └── {snapshot-checksum}.csv-metadata.json
            └── {snapshot-checksum}.csv
            └── …
        └── …
    └── …
```
