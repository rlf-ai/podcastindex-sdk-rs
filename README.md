# Rust SDK for PodcastIndex API

This repo contains the PodcastIndex SDK for Rust, more info here: https://podcastindex-org.github.io/docs-api

This code is generated by the OpenAPI Generator project: https://github.com/OpenAPITools/openapi-generator.

Notes:
* the generated code had to be modified to work with the PodcastIndex API (mostly around the authentication)
* the Rust SDK is not officially supported by PodcastIndex.


# Postman

A collection file for use in the [Postman](https://www.postman.com/) application is available for this API.

1. Download the contents of the [Postman Docs](https://github.com/Podcastindex-org/docs-api/tree/master/Postman%20Docs) folder.
2. Import the `PodcastIndex.postman_collection.json` collection to Postman
3. Import the `PodcastIndexOrgEnvironment.postman_environment.json` to Postman
4. Click \"Environments\" on the left sidebar
5. Select the checkbox next to the PodcastIndexOrgEnvironment entry
6. Set `AuthKey` and `SeceretKey` values under the \"Current Value\" column using your API information
7. Click \"Collections\" from the sidebar
8. Select PodcastIndex
9. Select and run the endpoint to test

# Contributing

The source for this API documentation is available at
[https://github.com/Podcastindex-org/docs-api](https://github.com/Podcastindex-org/docs-api).
Submit an Issue or create a Pull Request.

# Authentication Details

Sending an API request is easy. We use an Amazon-style request authorization token to secure each request.


Register for a free API key at https://api.podcastindex.org/


These headers parameters are required for each request: `User-Agent`, `X-Auth-Date`, `X-Auth-Key`, `Authorization`


See [Authentication](#auth) for description of parameters.


## Installation

TODO:

## Documentation for API Endpoints

All URIs are relative to *https://api.podcastindex.org/api/1.0*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AddApi* | [**add_slash_byfeedurl_get**](docs/AddApi.md#add_slash_byfeedurl_get) | **GET** /add/byfeedurl | By Feed URL
*AddApi* | [**add_slash_byfeedurl_post**](docs/AddApi.md#add_slash_byfeedurl_post) | **POST** /add/byfeedurl | By Feed URL
*AddApi* | [**add_slash_byitunesid_get**](docs/AddApi.md#add_slash_byitunesid_get) | **GET** /add/byitunesid | By iTunes ID
*AddApi* | [**add_slash_byitunesid_post**](docs/AddApi.md#add_slash_byitunesid_post) | **POST** /add/byitunesid | By iTunes ID
*AppleReplacementApi* | [**lookup**](docs/AppleReplacementApi.md#lookup) | **GET** /lookup | Lookup
*AppleReplacementApi* | [**search**](docs/AppleReplacementApi.md#search) | **GET** /search | Search
*CategoriesApi* | [**categories_slash_list**](docs/CategoriesApi.md#categories_slash_list) | **GET** /categories/list | List
*EpisodesApi* | [**episodes_slash_byfeedid**](docs/EpisodesApi.md#episodes_slash_byfeedid) | **GET** /episodes/byfeedid | By Feed ID
*EpisodesApi* | [**episodes_slash_byfeedurl**](docs/EpisodesApi.md#episodes_slash_byfeedurl) | **GET** /episodes/byfeedurl | By Feed URL
*EpisodesApi* | [**episodes_slash_byguid**](docs/EpisodesApi.md#episodes_slash_byguid) | **GET** /episodes/byguid | By GUID
*EpisodesApi* | [**episodes_slash_byid**](docs/EpisodesApi.md#episodes_slash_byid) | **GET** /episodes/byid | By ID
*EpisodesApi* | [**episodes_slash_byitunesid**](docs/EpisodesApi.md#episodes_slash_byitunesid) | **GET** /episodes/byitunesid | By iTunes ID
*EpisodesApi* | [**episodes_slash_bypodcastguid**](docs/EpisodesApi.md#episodes_slash_bypodcastguid) | **GET** /episodes/bypodcastguid | By Podcast GUID
*EpisodesApi* | [**episodes_slash_live**](docs/EpisodesApi.md#episodes_slash_live) | **GET** /episodes/live | Live
*EpisodesApi* | [**episodes_slash_random**](docs/EpisodesApi.md#episodes_slash_random) | **GET** /episodes/random | Random
*HubApi* | [**hub_slash_pubnotify**](docs/HubApi.md#hub_slash_pubnotify) | **GET** /hub/pubnotify | Pub Notify
*PodcastsApi* | [**podcasts_slash_byfeedid**](docs/PodcastsApi.md#podcasts_slash_byfeedid) | **GET** /podcasts/byfeedid | By Feed ID
*PodcastsApi* | [**podcasts_slash_byfeedurl**](docs/PodcastsApi.md#podcasts_slash_byfeedurl) | **GET** /podcasts/byfeedurl | By Feed URL
*PodcastsApi* | [**podcasts_slash_byguid**](docs/PodcastsApi.md#podcasts_slash_byguid) | **GET** /podcasts/byguid | By GUID
*PodcastsApi* | [**podcasts_slash_byitunesid**](docs/PodcastsApi.md#podcasts_slash_byitunesid) | **GET** /podcasts/byitunesid | By iTunes ID
*PodcastsApi* | [**podcasts_slash_bymedium**](docs/PodcastsApi.md#podcasts_slash_bymedium) | **GET** /podcasts/bymedium | By Medium
*PodcastsApi* | [**podcasts_slash_bytag**](docs/PodcastsApi.md#podcasts_slash_bytag) | **GET** /podcasts/bytag | By Tag
*PodcastsApi* | [**podcasts_slash_dead**](docs/PodcastsApi.md#podcasts_slash_dead) | **GET** /podcasts/dead | Dead
*PodcastsApi* | [**podcasts_slash_trending**](docs/PodcastsApi.md#podcasts_slash_trending) | **GET** /podcasts/trending | Trending
*RecentApi* | [**recent_slash_data**](docs/RecentApi.md#recent_slash_data) | **GET** /recent/data | Recent Data
*RecentApi* | [**recent_slash_episodes**](docs/RecentApi.md#recent_slash_episodes) | **GET** /recent/episodes | Episodes
*RecentApi* | [**recent_slash_feeds**](docs/RecentApi.md#recent_slash_feeds) | **GET** /recent/feeds | Feeds
*RecentApi* | [**recent_slash_newfeeds**](docs/RecentApi.md#recent_slash_newfeeds) | **GET** /recent/newfeeds | New Feeds
*RecentApi* | [**recent_slash_newvaluefeeds**](docs/RecentApi.md#recent_slash_newvaluefeeds) | **GET** /recent/newvaluefeeds | New Value Feeds
*RecentApi* | [**recent_slash_soundbites**](docs/RecentApi.md#recent_slash_soundbites) | **GET** /recent/soundbites | Soundbites
*SearchApi* | [**search_slash_byperson**](docs/SearchApi.md#search_slash_byperson) | **GET** /search/byperson | Search Episodes by Person
*SearchApi* | [**search_slash_byterm**](docs/SearchApi.md#search_slash_byterm) | **GET** /search/byterm | Search Podcasts
*SearchApi* | [**search_slash_bytitle**](docs/SearchApi.md#search_slash_bytitle) | **GET** /search/bytitle | Search Podcasts by Title
*SearchApi* | [**search_slash_music_slash_byterm**](docs/SearchApi.md#search_slash_music_slash_byterm) | **GET** /search/music/byterm | Search Music Podcasts
*StatsApi* | [**stats_slash_current**](docs/StatsApi.md#stats_slash_current) | **GET** /stats/current | Current
*ValueApi* | [**value_slash_batch_slash_byepisodeguid**](docs/ValueApi.md#value_slash_batch_slash_byepisodeguid) | **POST** /value/batch/byepisodeguid | Batch By Episode GUID
*ValueApi* | [**value_slash_byepisodeguid**](docs/ValueApi.md#value_slash_byepisodeguid) | **GET** /value/byepisodeguid | By Episode GUID
*ValueApi* | [**value_slash_byfeedid**](docs/ValueApi.md#value_slash_byfeedid) | **GET** /value/byfeedid | By Feed ID
*ValueApi* | [**value_slash_byfeedurl**](docs/ValueApi.md#value_slash_byfeedurl) | **GET** /value/byfeedurl | By Feed URL
*ValueApi* | [**value_slash_bypodcastguid**](docs/ValueApi.md#value_slash_bypodcastguid) | **GET** /value/bypodcastguid | By Feed GUID


## Documentation For Models

- [AddByfeedurlGet200Response](docs/AddByfeedurlGet200Response.md)
- [CategoriesList200Response](docs/CategoriesList200Response.md)
- [Data](docs/Data.md)
- [DestinationV4v](docs/DestinationV4v.md)
- [EpisodeObject](docs/EpisodeObject.md)
- [EpisodeType](docs/EpisodeType.md)
- [EpisodesByfeedid200Response](docs/EpisodesByfeedid200Response.md)
- [EpisodesByfeedid200ResponseQuery](docs/EpisodesByfeedid200ResponseQuery.md)
- [EpisodesByfeedurl200Response](docs/EpisodesByfeedurl200Response.md)
- [EpisodesByguid200Response](docs/EpisodesByguid200Response.md)
- [EpisodesByid200Response](docs/EpisodesByid200Response.md)
- [EpisodesByitunesid200Response](docs/EpisodesByitunesid200Response.md)
- [EpisodesLive200Response](docs/EpisodesLive200Response.md)
- [EpisodesRandom200Response](docs/EpisodesRandom200Response.md)
- [Existed](docs/Existed.md)
- [ExplicitEpisode](docs/ExplicitEpisode.md)
- [FeedBytag](docs/FeedBytag.md)
- [FeedCategories](docs/FeedCategories.md)
- [FeedDead](docs/FeedDead.md)
- [FeedItunes](docs/FeedItunes.md)
- [FeedPodcast](docs/FeedPodcast.md)
- [FeedSearch](docs/FeedSearch.md)
- [FeedTrending](docs/FeedTrending.md)
- [FeedsDataObj](docs/FeedsDataObj.md)
- [FeedsRecentInner](docs/FeedsRecentInner.md)
- [FeedsRecentNewInner](docs/FeedsRecentNewInner.md)
- [FeedsRecentValueInner](docs/FeedsRecentValueInner.md)
- [Funding](docs/Funding.md)
- [ItemItunesId](docs/ItemItunesId.md)
- [ItemPodcast](docs/ItemPodcast.md)
- [ItemPodcastLive](docs/ItemPodcastLive.md)
- [ItemPodcastRandom](docs/ItemPodcastRandom.md)
- [ItemPodcastRecent](docs/ItemPodcastRecent.md)
- [ItemSearchByperson](docs/ItemSearchByperson.md)
- [ItemsDataObj](docs/ItemsDataObj.md)
- [ItemsSoundbitesInner](docs/ItemsSoundbitesInner.md)
- [ItunesResults](docs/ItunesResults.md)
- [LiveitemPodcast](docs/LiveitemPodcast.md)
- [Locked](docs/Locked.md)
- [ModelV4v](docs/ModelV4v.md)
- [Person](docs/Person.md)
- [PodcastsByfeedid200Response](docs/PodcastsByfeedid200Response.md)
- [PodcastsByfeedurl200Response](docs/PodcastsByfeedurl200Response.md)
- [PodcastsByguid200Response](docs/PodcastsByguid200Response.md)
- [PodcastsByitunesid200Response](docs/PodcastsByitunesid200Response.md)
- [PodcastsBymedium200Response](docs/PodcastsBymedium200Response.md)
- [PodcastsBytag200Response](docs/PodcastsBytag200Response.md)
- [PodcastsDead200Response](docs/PodcastsDead200Response.md)
- [PodcastsTrending200Response](docs/PodcastsTrending200Response.md)
- [ProtocolSocialInteract](docs/ProtocolSocialInteract.md)
- [QueryByepisodeguid](docs/QueryByepisodeguid.md)
- [QueryGuid](docs/QueryGuid.md)
- [QueryGuidId](docs/QueryGuidId.md)
- [QueryGuids](docs/QueryGuids.md)
- [QueryId](docs/QueryId.md)
- [QueryUrl](docs/QueryUrl.md)
- [RecentData200Response](docs/RecentData200Response.md)
- [RecentEpisodes200Response](docs/RecentEpisodes200Response.md)
- [RecentFeeds200Response](docs/RecentFeeds200Response.md)
- [RecentNewfeeds200Response](docs/RecentNewfeeds200Response.md)
- [RecentNewvaluefeeds200Response](docs/RecentNewvaluefeeds200Response.md)
- [RecentSoundbites200Response](docs/RecentSoundbites200Response.md)
- [Search200Response](docs/Search200Response.md)
- [Search400Response](docs/Search400Response.md)
- [SearchByperson200Response](docs/SearchByperson200Response.md)
- [SearchByterm200Response](docs/SearchByterm200Response.md)
- [SocialInteractItem](docs/SocialInteractItem.md)
- [Soundbite](docs/Soundbite.md)
- [Stats](docs/Stats.md)
- [StatsCurrent200Response](docs/StatsCurrent200Response.md)
- [Status](docs/Status.md)
- [StatusFeed](docs/StatusFeed.md)
- [StatusLive](docs/StatusLive.md)
- [Transcript](docs/Transcript.md)
- [Type](docs/Type.md)
- [TypeDestination](docs/TypeDestination.md)
- [TypeV4v](docs/TypeV4v.md)
- [Value](docs/Value.md)
- [ValueBatchByepisodeguid200Response](docs/ValueBatchByepisodeguid200Response.md)
- [ValueByepisodeguid](docs/ValueByepisodeguid.md)
- [ValueByepisodeguid200Response](docs/ValueByepisodeguid200Response.md)
- [ValueByepisodeguidBatch](docs/ValueByepisodeguidBatch.md)
- [ValueByfeedid200Response](docs/ValueByfeedid200Response.md)
- [ValueByfeedurl200Response](docs/ValueByfeedurl200Response.md)
- [ValueBypodcastguid200Response](docs/ValueBypodcastguid200Response.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```
