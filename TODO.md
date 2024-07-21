# TODOs

## Architecture

### UI

- [x] Better reactive design
    - [x] Select should preserve scroll
- [x] Filter media in selector by query
- [x] Media detail on hover
- [ ] Toaster notifications
    - [ ] new media
- [x] Uploads
    
### API

- [ ] Persist storage on server
    - [x] storage abstraction
    - [x] in-mem kv storage
    - [ ] on-disk kv storage
    - [ ] sqlite storage
    - [ ] garbage collection instead of deletion
- [ ] Improve metadata automation
    - [x] `ffprobe` to mine metadata
    - ~~[ ] Use collections for nested directories~~
    - [x] Recurse directories for media discovery
    
## Maintenance

- [x] Split resumable uploads into a separate crate
- [x] API should return schema-appropriate media URLs
    - [x] Use env var in API to set media server url
- [ ] fix `/` going to catch-all route

## Greenfield

- [ ] Job manager
    - [ ] Display `ffmpeg` progress
    - [ ] Resumable conversions
    - [ ] Confirmation for anything destructive
- [ ] Upload manager
    - [ ] Resumable
    - [ ] Upload progress bars
- [ ] Use object storage
- [ ] Alternative protocols (ws,quic)
- [ ] API tests

## PR

- [x] Dry demo 
- [ ] User docs
- [x] GitHub pages automation

## Under Consideration

- [x] better data model and sync procedure (CRDT?)
    - [x] database in browser storage
    - [x] only pull out-of-date items from server
    - [ ] ~~treat browser storage as write-back cache~~
- [ ] ~~Consider HTTP/2 for event streams~~
