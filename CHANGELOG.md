# Changelog
All notable changes to this project will be documented in this file.

## [0.1.1] - 2025-11-23

### Added
- Changelog file
- Event trait that lets you pass callback to get specific event, instead of matching event(), corresponding functions ```use_event```. ```use_exit```. ```use_custom_event```. ```use_page_change```
- Global callback handler

### Changed

- event function moved from router to trait and now called get_event. Returned value changed from ```&Option<Events<T::Ev>>``` to  ```&Events<T>```


## [0.1.0] - 2025-10-18

# Release!
