<script lang="ts">
  import { get } from 'svelte/store';
  import { sources, postings } from "../store"; 
	import { goto } from '$app/navigation';

  let storedPostings = get(postings);

  postings.subscribe((_) => {
    storedPostings = get(postings);
  });
</script>

<div class="w-2/3">
  {#each storedPostings as posting}
    <a href="/posting/{posting.id}">
      <div class="card w-full">
        <div class="card-body items-left text-left">
          <h2 class="card-title">{posting.title}</h2>
          <p class="text-justify">{posting.description}</p>
        </div>
      </div>
    </a>
  {/each}
</div>
