# Throughs

- this is using lsm trees
- crude imp of Bitcask storage backend
# Structure
```
┌─────────────────────┬───────────────────────┬─────────────┬───────────────┬──────────────┐
│ CRC (Checksum)      │ Timestamp (8 bytes)   │ Key Size    │ Value Size    │ Key + Value  │
└─────────────────────┴───────────────────────┴─────────────┴───────────────┴──────────────┘
```
droping timestamp for now
# indexing
A key-value in-memory index is built during runtime.
```
key -> (file_id, offset, value_size)
```
