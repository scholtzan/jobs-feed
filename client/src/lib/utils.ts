export function todaysPostings(postings) {
    let currentDate = new Date();
    return postings.filter((p) => {
        let postingDate = new Date(p.created_at);
        return postingDate.getFullYear() === currentDate.getFullYear() &&
        postingDate.getMonth() === currentDate.getMonth() &&
        postingDate.getDate() === currentDate.getDate()
    });
}

export function postingsForSource(postings) {
    return Object.groupBy(postings, ( p ) => p.source_id);
}
