---
sidebar_position: 5
---

# FAQ and Troubleshooting

## I have a suggestion for improvement or want to report a bug?

You can report issues, suggestions, or ideas for improvements [here](https://github.com/scholtzan/jobs-feed/issues).

## Is there a hosted version of Jobs Feed?

There is currently no hosted version of Jobs Feed. To use the application, it needs to run locally.

## Support for other LLM vendors?

Currently, Jobs Feed requires OpenAI credentials for its job extraction. Other LLM vendors are not supported at this time.

## Some jobs are failing to load.

If the source page is large and has many job postings, only a portion of them will be parsed and loaded. This is to reduce the amount of content sent to OpenAI. Performing a subsequent refresh will load older job postings that were previously not considered.

:::tip

**Have a question that didn't get answered?**

Please open a [Github issue](https://github.com/scholtzan/jobs-feed/issues) to get help.
