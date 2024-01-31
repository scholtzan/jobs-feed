<script lang="ts">
  import { get } from 'svelte/store';
  import { sources, postings } from "../store"; 

  let storedPostings = get(postings);

  postings.subscribe((_) => {
    storedPostings = get(postings);
  })
</script>

<div>
  {#each storedPostings as posting}
    <div class="card w-96 ">
      <div class="card-body items-left text-left">
        <h2 class="card-title">{posting.title}</h2>
        <p>{posting.description}</p>
      </div>
    </div>
  {/each}
</div>
