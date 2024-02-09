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
    } else {
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
          console.log(readPostings);
        } else {
            // todo: error
            console.log("Cannot mark postings as read");
        }
    });
  }

</script>

<div class="w-2/3">
  <h1 class="flex grow text-4xl font-bold py-8 px-4">
    {#if sourceSelected == "all"}
    All Job Postings
    {:else if sourceSelected == "today"} 
    Today's Job Postings
    {:else}
      {storedSources.find((s) => s.id == sourceSelected).name}
    {/if}

    <div class="flex flex-row grow justify-end px-4">
      <button class="btn btn-ghost btn-square" on:click={() => markAsRead(storedPostshownPostingsings.map((p) => p.id))}>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-9 h-9">
          <path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
        </svg>        
      </button>
    </div>
  </h1>
  {#each shownPostings as posting}
    <a href="/posting/{posting.id}" on:click={() => markAsRead([posting.id])}>
      <div class="card card-compact w-full {readPostings.has(posting.id) ? "text-slate-100" : ""}">
        <div class="card-body items-left text-left">
          <h2 class="card-title">{posting.title}</h2>
          <p class="text-justify">{posting.description}</p>
        </div>
      </div>
    </a>
  {/each}

  {#if shownPostings.length > 0}
  <div class="py-8 flex-none  px-4">
    <!-- todo -->
    <button on:click={() => markAsRead(shownPostings.map((p) => p.id))} class="btn btn-active w-full max-w">Mark All As Read</button>
  </div>
  {/if}
</div>
