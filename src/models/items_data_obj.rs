/*
 * PodcastIndex.org API
 *
 * Download the openapi spec in [json](pi_api.json) or [yaml](pi_api.yaml) format.  # Overview  The Podcast Index (Podcast Index LLC) is a software developer focused partnership that provides tools and data to anyone who aspires to create new and exciting Podcast experiences without the heavy lifting of indexing, aggregation and data management.  # Example Code  Users have provided example code for working the API in the following languages:    - [AWS Lambda - python](https://github.com/tbowers/python-podcastindex-org-lambda)   - [Bash](https://github.com/suorcd/Bash-podcastindex-org-example)   - [C#](https://github.com/ComicStrip/csharp-podcastindex-org-example)   - [Elisp](https://github.com/sabexx/elisp-example)   - [Flutter/Dart](https://github.com/crediblecreative/flutter-dart-podcastindex-org-example)   - [Go](https://github.com/ComicStrip/Go-podcastindex-org-example)   - [Go](https://github.com/kilobit/podcast-index-client)   - [Java](https://github.com/ComicStrip/Java-podcastindex-org-example)   - [Java](https://github.com/stucoates/PodcastIndexJavaClient)   - [Node.js](https://github.com/ComicStrip/node.js-podcastindex-org-example)   - [PHP](https://github.com/Podcastindex-org/example-code)   - [Python](https://github.com/tbowers/python-podcastindex-org-example)   - [Swift](https://github.com/ComicStrip/Swift-podcastindex-org-example)   - ... [More](https://github.com/Podcastindex-org-Examples)  Don't see your desired language, create an example repo and create a [Pull Request](https://github.com/Podcastindex-org/docs-api/pulls) with a link to your example code project!  # Libraries  User created libraries for working with the API:    - Java       - [podcast4j](https://github.com/yusufyilmazfr/podcast4j)   - .NET       - [PodcastIndexSharp](https://www.nuget.org/packages/PodcastIndexSharp)   - Node.js/npm/yarn       - [podcast-index-api](https://www.npmjs.com/package/podcast-index-api)       - [podcastdx-client](https://www.npmjs.com/package/podcastdx-client)   - PHP       - [podcastindex-php](https://github.com/LowSociety/podcastindex-php)   - Python/pip       - [python-podcastindex](https://pypi.org/project/python-podcastindex/)   - R       - [podindexr](https://github.com/rpodcast/podindexr)   - Ruby       - [podcast-index](https://github.com/jasonyork/podcast-index)   - Swift       - [PodcastIndexKit](https://github.com/SparrowTek/PodcastIndexKit)   - Kotlin       - [PodcastIndex-SDK](https://github.com/mr3y-the-programmer/PodcastIndex-SDK)  Are we missing a library? Did you create one for a different language? Create a [Pull Request](https://github.com/Podcastindex-org/docs-api/pulls) with a link to the new library!  # Postman  A collection file for use in the [Postman](https://www.postman.com/) application is available for this API.    1. Download the contents of the [Postman Docs](https://github.com/Podcastindex-org/docs-api/tree/master/Postman%20Docs) folder.   2. Import the `PodcastIndex.postman_collection.json` collection to Postman   3. Import the `PodcastIndexOrgEnvironment.postman_environment.json` to Postman   4. Click \"Environments\" on the left sidebar   5. Select the checkbox next to the PodcastIndexOrgEnvironment entry   6. Set `AuthKey` and `SeceretKey` values under the \"Current Value\" column using your API information    7. Click \"Collections\" from the sidebar   8. Select PodcastIndex   9. Select and run the endpoint to test  # Contributing  The source for this API documentation is available at [https://github.com/Podcastindex-org/docs-api](https://github.com/Podcastindex-org/docs-api). Submit an Issue or create a Pull Request.  # Authentication Details  Sending an API request is easy. We use an Amazon-style request authorization token to secure each request.   Register for a free API key at https://api.podcastindex.org/   These headers parameters are required for each request: `User-Agent`, `X-Auth-Date`, `X-Auth-Key`, `Authorization`   See [Authentication](#auth) for description of parameters.  # Legal  Legal    - [Privacy Policy](https://github.com/Podcastindex-org/legal/blob/main/PrivacyPolicy.md)   - [Terms of Service](https://github.com/Podcastindex-org/legal/blob/main/TermsOfService.md)   - [License](https://github.com/Podcastindex-org/docs-api/blob/master/LICENSE)
 *
 * The version of the OpenAPI document: 1.12.1
 * Contact: info@podcastindex.org
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemsDataObj {
    /// The internal PodcastIndex.org episode ID.
    #[serde(rename = "episodeId", skip_serializing_if = "Option::is_none")]
    pub episode_id: Option<i32>,
    /// Name of the feed
    #[serde(rename = "episodeTitle", skip_serializing_if = "Option::is_none")]
    pub episode_title: Option<String>,
    /// The item-level description of the episode.   Uses the longer of the possible fields in the feed: `<description>`, `<itunes:summary>` and `<content:encoded>`
    #[serde(rename = "episodeDescription", skip_serializing_if = "Option::is_none")]
    pub episode_description: Option<String>,
    /// The item-level image for the episode
    #[serde(rename = "episodeImage", skip_serializing_if = "Option::is_none")]
    pub episode_image: Option<String>,
    /// The date and time the episode was published
    #[serde(rename = "episodeTimestamp", skip_serializing_if = "Option::is_none")]
    pub episode_timestamp: Option<i32>,
    /// The time the episode was added to the index
    #[serde(rename = "episodeAdded", skip_serializing_if = "Option::is_none")]
    pub episode_added: Option<i32>,
    /// URL/link to the episode file
    #[serde(
        rename = "episodeEnclosureUrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_enclosure_url: Option<String>,
    /// The length of the item specified by the `enclosureUrl` in bytes
    #[serde(
        rename = "episodeEnclosureLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_enclosure_length: Option<i32>,
    /// The Content-Type for the item specified by the `enclosureUrl`
    #[serde(
        rename = "episodeEnclosureType",
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_enclosure_type: Option<String>,
    /// The estimated length of the item specified by the `enclosureUrl` in seconds. Will be null for `liveItem`.
    #[serde(
        rename = "episodeDuration",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_duration: Option<Option<i32>>,
    #[serde(
        rename = "episodeType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_type: Option<Option<models::EpisodeType>>,
    /// The internal PodcastIndex.org Feed ID.
    #[serde(rename = "feedId", skip_serializing_if = "Option::is_none")]
    pub feed_id: Option<i32>,
}

impl ItemsDataObj {
    pub fn new() -> ItemsDataObj {
        ItemsDataObj {
            episode_id: None,
            episode_title: None,
            episode_description: None,
            episode_image: None,
            episode_timestamp: None,
            episode_added: None,
            episode_enclosure_url: None,
            episode_enclosure_length: None,
            episode_enclosure_type: None,
            episode_duration: None,
            episode_type: None,
            feed_id: None,
        }
    }
}
