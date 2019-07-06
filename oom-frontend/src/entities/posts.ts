export type NewPost = {
    title: string,
    body: string,
    question_post_id: number | null,
};

export type UpdateBestAnswer = {
    is_best_answer: boolean | null,
};

export type UpdateContent = {
    title: string | null,
    body: string | null,
};
