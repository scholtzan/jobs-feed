<script lang="ts">
  import { get } from 'svelte/store';
  import { sources, postings, selectedSource } from "../store"; 
  import { todaysPostings, postingsForSource } from "../utils";
	import { goto } from '$app/navigation';

  let storedPostings = get(postings);
  let sourceSelected = get(selectedSource);
  let storedSources = get(sources);
  let shownPostings = storedPostings;

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

</script>

<div class="w-2/3">
  <h1 class="text-4xl font-bold py-8 px-4">
    {#if sourceSelected == "all"}
    All Job Postings
    {:else if sourceSelected == "today"} 
    Today's Job Postings
    {:else}
      {storedSources.find((s) => s.id == sourceSelected).name}
    {/if}
  </h1>
  {#each shownPostings as posting}
    <a href="/posting/{posting.id}">
      <div class="card card-compact w-full">
        <div class="card-body items-left text-left">
          <h2 class="card-title">{posting.title}</h2>
          <p class="text-justify">{posting.description}</p>
        </div>
      </div>
    </a>
  {/each}
</div>
