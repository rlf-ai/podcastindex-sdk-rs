# \StatsApi

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stats_slash_current**](StatsApi.md#stats_slash_current) | **GET** /stats/current | Current



## stats_slash_current

> models::StatsCurrent200Response stats_slash_current(pretty)
Current

Return the most recent index statistics.   Hourly statistics can also be access at https://stats.podcastindex.org/daily_counts.json   Example: https://api.podcastindex.org/api/1.0/stats/current?pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::StatsCurrent200Response**](stats_current_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

