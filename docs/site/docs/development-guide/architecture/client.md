---
sidebar_position: 2
---

# Client

The client code is located in the `client/` directory and serves as the web interface that users interact with. It is implemented in TypeScript using [SvelteKit](https://kit.svelte.dev/) and [DaisyUI](https://daisyui.com/), with icons provided by [heroicons](https://heroicons.com/).

## Routes

The different side panes and dialogs are implemented as individual routes, allowing each dialog to have a custom URL. The routes and the structure of the pages are implemented in the `routes/` directory.

## Components

Pages can reference reusable components, such as inputs with built-in validation. These components are located in the `components/` directory.

## API

To send requests to the server, a thin wrapper that encodes all the API functionality is located in the `api/` directory. A separate class with functions for each endpoint is available. Any request response includes information about whether the request was successful or not (e.g., in case of a server error) as well as response data. The response data can be converted into specific TypeScript types defined in `types/`.

## Shared State

Jobs Feed uses [Svelte stores](https://kit.svelte.dev/docs/state-management#using-stores-with-context) to share data across different pages and components. These stores are accessed only via specific type instances.

