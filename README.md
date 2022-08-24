# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.3.0
- Package version: 0.3.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CrateApi* | [**handle_add_channel**](docs/CrateApi.md#handle_add_channel) | **POST** /add_channel | Add channel
*CrateApi* | [**handle_get_channels**](docs/CrateApi.md#handle_get_channels) | **GET** /notify/get_channels/{user_id} | Get notification channels for user
*CrateApi* | [**handle_get_telegram_chat_id**](docs/CrateApi.md#handle_get_telegram_chat_id) | **POST** /get_telegram_chat_id | Get the chat ID of a telegram username
*CrateApi* | [**handle_notify**](docs/CrateApi.md#handle_notify) | **POST** /notify | Send notification
*CrateApi* | [**handle_remove_channel**](docs/CrateApi.md#handle_remove_channel) | **POST** /remove_channel | Remove channel


## Documentation For Models

 - [AddChannelBody](docs/AddChannelBody.md)
 - [ChannelsResponse](docs/ChannelsResponse.md)
 - [GetTelegramChatIdBody](docs/GetTelegramChatIdBody.md)
 - [NotifyBody](docs/NotifyBody.md)
 - [RemoveChannelBody](docs/RemoveChannelBody.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



