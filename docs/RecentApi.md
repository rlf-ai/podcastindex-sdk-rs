# \RecentApi

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**recent_slash_data**](RecentApi.md#recent_slash_data) | **GET** /recent/data | Recent Data
[**recent_slash_episodes**](RecentApi.md#recent_slash_episodes) | **GET** /recent/episodes | Episodes
[**recent_slash_feeds**](RecentApi.md#recent_slash_feeds) | **GET** /recent/feeds | Feeds
[**recent_slash_newfeeds**](RecentApi.md#recent_slash_newfeeds) | **GET** /recent/newfeeds | New Feeds
[**recent_slash_newvaluefeeds**](RecentApi.md#recent_slash_newvaluefeeds) | **GET** /recent/newvaluefeeds | New Value Feeds
[**recent_slash_soundbites**](RecentApi.md#recent_slash_soundbites) | **GET** /recent/soundbites | Soundbites



## recent_slash_data

> models::RecentData200Response recent_slash_data(max, since, pretty)
Recent Data

This call returns every new feed and episode added to the index over the past 24 hours in reverse chronological order.   This is similar to `/recent/feeds` but uses the date the feed was found by the index rather than the feed's internal timestamp.   Similar data can also be accessed using object storage root url https://tracking.podcastindex.org/current   Examples:    - https://api.podcastindex.org/api/1.0/recent/data?pretty   - https://api.podcastindex.org/api/1.0/recent/data?pretty&max=10   - https://api.podcastindex.org/api/1.0/recent/data?pretty&max=10&since=1671164867 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Maximum number of results to return (includes both `feeds` and `items`).  |  |[default to 1000]
**since** | Option<**i32**> | Return items since the specified epoch timestamp.  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::RecentData200Response**](recent_data_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recent_slash_episodes

> models::RecentEpisodes200Response recent_slash_episodes(max, exclude_string, before, fulltext, pretty)
Episodes

This call returns the most recent `max` number of episodes globally across the whole index, in reverse chronological order.   Example: https://api.podcastindex.org/api/1.0/recent/episodes?max=7&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Maximum number of results to return.  |  |[default to 10]
**exclude_string** | Option<**String**> | Any item containing this string will be discarded from the result set.   This may, in certain cases, reduce your set size below your `max` value.   Matches against the `title` and URL properties.  |  |
**before** | Option<**i32**> | If you pass a PodcastIndex Episode ID, you will get recent episodes before that ID, allowing you to walk back through the episode history sequentially.  |  |
**fulltext** | Option<**bool**> | If present, return the full text value of any text fields (ex: `description`). If not provided, field value is truncated to 100 words.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::RecentEpisodes200Response**](recent_episodes_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recent_slash_feeds

> models::RecentFeeds200Response recent_slash_feeds(max, since, lang, cat, notcat, pretty)
Feeds

This call returns the most recent `max` feeds, in reverse chronological order.   Examples:    - https://api.podcastindex.org/api/1.0/recent/feeds?pretty   - https://api.podcastindex.org/api/1.0/recent/feeds?max=20&cat=102,health&lang=de,ja&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Maximum number of results to return.  |  |[default to 40]
**since** | Option<**i32**> | Return items since the specified epoch timestamp.  |  |
**lang** | Option<**String**> | Specifying a language code (like \"en\") will return only episodes having that specific language.   You can specify multiple languages by separating them with commas.   If you also want to return episodes that have no language given, use the token \"unknown\". (ex. en,es,ja,unknown).   Values are not case sensitive.  |  |
**cat** | Option<**String**> | Use this argument to specify that you **ONLY** want episodes with these categories in the results.   Separate multiple categories with commas.   You may specify either the Category ID and/or the Category Name.   Values are not case sensitive.   The `cat` and `notcat` filters can be used together to fine tune a very specific result set.   Category numbers and names can be found in the [Podcast Namespace documentation](https://github.com/Podcastindex-org/podcast-namespace/blob/main/categories.json)  |  |
**notcat** | Option<**String**> | Use this argument to specify categories of episodes to **NOT** show in the results.   Separate multiple categories with commas.   You may specify either the Category ID and/or the Category Name.   Values are not case sensitive.   The `cat` and `notcat` filters can be used together to fine tune a very specific result set.   Category numbers and names can be found in the [Podcast Namespace documentation](https://github.com/Podcastindex-org/podcast-namespace/blob/main/categories.json)  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::RecentFeeds200Response**](recent_feeds_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recent_slash_newfeeds

> models::RecentNewfeeds200Response recent_slash_newfeeds(max, since, feedid, desc, pretty)
New Feeds

This call returns every new feed added to the index over the past 24 hours in reverse chronological order.   Examples:    - https://api.podcastindex.org/api/1.0/recent/newfeeds?pretty   - https://api.podcastindex.org/api/1.0/recent/newfeeds?pretty&since=1613805000   - https://api.podcastindex.org/api/1.0/recent/newfeeds?feedid=2653471&pretty   - https://api.podcastindex.org/api/1.0/recent/newfeeds?feedid=2653471&desc&pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Maximum number of results to return.  |  |[default to 40]
**since** | Option<**i32**> | Return items since the specified epoch timestamp.  |  |
**feedid** | Option<**String**> | The PodcastIndex Feed ID to start from (or go to if `desc` specified).   If `since` parameter also specified, value of `since` is ignored.  |  |
**desc** | Option<**bool**> | If present, display feeds in descending order.   Only applicable when using `feedid` parameter.   Parameter shall not have a value  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::RecentNewfeeds200Response**](recent_newfeeds_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recent_slash_newvaluefeeds

> models::RecentNewvaluefeeds200Response recent_slash_newvaluefeeds(max, since, pretty)
New Value Feeds

This call returns feeds that have added a `value` tag in reverse chronological order.   Example: https://api.podcastindex.org/api/1.0/recent/newvaluefeeds?pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Maximum number of results to return.  |  |[default to 40]
**since** | Option<**i32**> | Return items since the specified epoch timestamp.  |  |
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::RecentNewvaluefeeds200Response**](recent_newvaluefeeds_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recent_slash_soundbites

> models::RecentSoundbites200Response recent_slash_soundbites(max, pretty)
Soundbites

This call returns the most recent `max` soundbites that the index has discovered.   A soundbite consists of an enclosure url, a start time and a duration. It is documented in the [podcast namespace](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#soundbite).   Example: https://api.podcastindex.org/api/1.0/recent/soundbites?pretty 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | Option<**i32**> | Maximum number of soundbites to return.  |  |[default to 60]
**pretty** | Option<**bool**> | If present, makes the output “pretty” to help with debugging.   Parameter shall not have a value  |  |

### Return type

[**models::RecentSoundbites200Response**](recent_soundbites_200_response.md)

### Authorization

[Authorization](../README.md#Authorization), [User-Agent](../README.md#User-Agent), [Date](../README.md#Date), [API-Key](../README.md#API-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

