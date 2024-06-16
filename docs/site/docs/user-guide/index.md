---
sidebar_position: 3
---

# User Guide

## Preferences

Jobs Feed requires an OpenAI API key, which can be found on the [OpenAI API keys page](https://platform.openai.com/api-keys).
This API key will be used to extract relevant job postings from career pages.

Copy the API key and go to the [preferences page in Jobs Feed](http://127.0.0.1:3000/preferences/). Insert the API key and select the preferred [OpenAI model](https://platform.openai.com/docs/models).

<div style={{width: '600px'}}>
![Settings dialog in Jobs Feed](/img/settings.png)
</div>

## Adding Sources

Sources are career websites or other URLs from which job postings are extracted. New sources can be added using the toolbar button:

<div style={{width: '200px'}}>
![Add a new source](/img/add-source.png)
</div>

1. **Provide Source Details:** Enter a name for the new source and the URL.
2. **Configure Advanced Settings (Optional):**
    * **Pagination:** Specify a CSS selector for pagination elements, such as a "Next" button. This ensures that job postings spread across multiple pages are fully extracted.
    * **Element Selector:** Provide a CSS selector for specific page elements to narrow down the content that needs to be searched for job postings, improving extraction speed and efficiency.
3. **Custom Favicon URL (Optional):** Click on the icon next to "New Source" and provide a URL to a favicon. This is useful if the source page is hosted under a different domain than the company website.

<div style={{width: '600px'}}>
![Add a new source](/img/new-source.png)
</div>


## Adding Filters

To limit the number of job postings and ensure the most relevant ones are being extracted, filters can be configured:

<div style={{width: '200px'}}>
![Add filters](/img/add-filter.png)
</div>

Specify desired conditions a job posting should meet, such as relevant job titles, preferred skills or specific locations.

<div style={{width: '600px'}}>
![Configure filters](/img/new-filter.png)
</div>

## Managing Postings

To search for new job postings, click the "Refresh Postings" button in the toolbar.

<div style={{width: '300px'}}>
![Refresh postings](/img/refresh-postings.png)
</div>

* **Search All Sources:** This action will initiate a search across all configured sources for new job postings. The duration of this process may vary depending on the number of sources being searched. A loading indicator next to each source will show if the search is still in progress.
* **Refresh Specific Sources:** Individual sources can be refreshed through the context menu for more targeted updates.

Once the search is complete, the available postings will be listed and are ready for review:

To search for new job postings, click on the "Refresh Postings" button in the toolbar.

<div style={{width: '600px'}}>
![List of postings](/img/postings.png)
</div>

* **Mark as Read:** Indicate which postings were already viewed. Clicking on posting details will automatically mark them as read.
* **Bookmark for Later:** Save interesting postings to review them at a later time.
* **Rate Postings:** Use the thumbs up and down buttons to indicate preference. This will help the application suggest more or fewer similar postings in the future.
* **View Details:** Click on a posting to see more details, if available. A link to the original posting on the source page is also provided.

<div style={{width: '600px'}}>
![Posting details](/img/posting-details.png)
</div>

Next to the list of postings appear suggestions for companies similar to the currently selected one to help discovering new, relevant sources.

