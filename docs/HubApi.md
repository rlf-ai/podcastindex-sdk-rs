# \HubApi

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hub_slash_pubnotify**](HubApi.md#hub_slash_pubnotify) | **GET** /hub/pubnotify | Pub Notify



## hub_slash_pubnotify

> models::Search400Response hub_slash_pubnotify(id, url, pretty)
Pub Notify

Notify the index that a feed has changed   Note: No API key needed for this endpoint.   Examples:    - https://api.podcastindex.org/api/1.0/hub/pubnotify?id=920666&pretty   - https://api.podcastindex.org/api/1.0/hub/pubnotify?url=https://feeds.theincomparable.com/batmanuniversity&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**i32**> | The PodcastIndex Feed ID   The `id` or the `url` is required.  |  |
**url** | Option<**String**> | Podcast feed URL   The `id` or the `url` is required.  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::Search400Response**](search_400_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

