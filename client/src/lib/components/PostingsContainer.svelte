<script lang="ts">
  import { get } from 'svelte/store';
  import { sources, postings, selectedSource } from "../store"; 
  import { todaysPostings, postingsForSource } from "../utils";
	import { goto } from '$app/navigation';

  let storedPostings = get(postings);
  let sourceSelected = get(selectedSource);
  let storedSources = get(sources);
  let shownPostings = storedPostings;
  let readPostings = new Set();

  postings.subscribe((_) => {
    storedPostings = get(postings);
  });

  selectedSource.subscribe((_) => {
    sourceSelected = get(selectedSource);
    if (sourceSelected == "all") {
      shownPostings = storedPostings;
    } else if (sourceSelected == "today") {
      shownPostings = todaysPostings(storedPostings);
    } else if (sourceSelected == "bookmarked") {
      const res = fetch("/postings/bookmarked", {
        method: "GET"
      }).then((response) => {
        if (response.status == 200) {
          response.json().then((json) => {
            shownPostings = json as Posting[];
          });
        } else {
          console.log("Could not get postings");
          shownPostings = [];
        }
      })
    }
     else {
      let postingsPerSource = postingsForSource(storedPostings);
      if (sourceSelected in postingsPerSource) {
        shownPostings = postingsForSource(storedPostings)[sourceSelected];
      } else {
        shownPostings = [];
      }
    }
  });

  sources.subscribe((_) => {
    storedSources = get(sources);
  });

  function markAsRead(ids) {
    const res = fetch('/postings/mark_read', {
        method: 'PUT',
        body: JSON.stringify(ids)
    }).then((response) => {
        if (response.status == 200) {
          readPostings = new Set([...readPostings, ...ids]);
        } else {
            // todo: error
            console.log("Cannot mark postings as read");
        }
    });
  }

  function bookmark(id) {
    let posting = shownPostings.find((p) => p.id == id);
    posting.bookmarked = !posting.bookmarked;
    shownPostings = shownPostings;

    const res = fetch('/postings/' + id, {
        method: 'PUT',
        body: JSON.stringify(posting)
    }).then((response) => {
        if (response.status == 200) {
          console.log("updated bookmark")
        } else {
          console.log("Could not update bookmarking for posting");
          posting.bookmarked = !posting.bookmarked;
          shownPostings = shownPostings;
        }
    });
  }

</script>

<div class="w-2/3">
  <div class="flex flex-row grow justify-end px-4">
    <button class="btn btn-ghost btn-square" on:click={() => markAsRead(shownPostings.map((p) => p.id))}>
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-9 h-9">
        <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
      </svg>        
    </button>
  </div>
  <h1 class="flex grow text-4xl font-bold py-8 px-4">
    {#if sourceSelected == "all"}
    All Job Postings
    {:else if sourceSelected == "today"} 
    Today's Job Postings
    {:else if sourceSelected == "bookmarked"}
    Bookmarked Postings
    {:else}
      {storedSources.find((s) => s.id == sourceSelected).name}
    {/if}
  </h1>
  {#each shownPostings as posting}
    <div class="card card-compact w-full group {readPostings.has(posting.id) && selectedSource != "bookmarked" ? "text-slate-100" : ""}">
      <div class="card-body items-left text-left">
        <div class="flex flex-row grow">
          <a href="/posting/{posting.id}" on:click={() => markAsRead([posting.id])}>
            <h2 class="card-title flex grow mb-0">
              {posting.title}
            </h2>
          </a>

          <div class="flex flex-row grow justify-end px-4 gap-2">
            <button class="btn btn-ghost btn-square btn-xs hidden group-hover:block" on:click={() => markAsRead([posting.id])}>
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
              </svg>        
            </button>
            <button class="btn btn-ghost btn-square btn-xs hidden group-hover:block" on:click={() => bookmark(posting.id)}>
              {#if posting.bookmarked}
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
                <path fill-rule="evenodd" d="M6.32 2.577a49.255 49.255 0 0 1 11.36 0c1.497.174 2.57 1.46 2.57 2.93V21a.75.75 0 0 1-1.085.67L12 18.089l-7.165 3.583A.75.75 0 0 1 3.75 21V5.507c0-1.47 1.073-2.756 2.57-2.93Z" clip-rule="evenodd" />
              </svg>
              {:else}                
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0 1 11.186 0Z" />
              </svg>
              {/if}                  
            </button>
          </div>
        </div>
        <a href="/posting/{posting.id}" on:click={() => markAsRead([posting.id])}>
          <p class="text-justify">{posting.description}</p>
        </a>
      </div>
    </div>
  {/each}

  {#if shownPostings.length > 0}
  <div class="py-8 flex-none px-4">
    <!-- todo -->
    <button on:click={() => markAsRead(shownPostings.map((p) => p.id))} class="btn btn-active w-full max-w">Mark All As Read</button>
  </div>
  {/if}
</div>
