# \CategoriesApi

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**categories_slash_list**](CategoriesApi.md#categories_slash_list) | **GET** /categories/list | List



## categories_slash_list

> models::CategoriesList200Response categories_slash_list(pretty)
List

Return all the possible categories supported by the index.   Example: https://api.podcastindex.org/api/1.0/categories/list?pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::CategoriesList200Response**](categories_list_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

