import axios, { AxiosInstance, AxiosResponse } from 'axios';
import { NewPost, UpdateBestAnswer, UpdateContent } from '../entities/posts';
import { NewComment } from '../entities/comments';
import { QuestionsQuery, AnswersQuestionsQuery } from '../entities/questions';
import { AnswersQuery } from '../entities/answers';
import { TagsQuery, TagsSearchQuery, NewTag } from '../entities/tags';
import { UsersQuery } from '../entities/users';
import { NewUp } from '../entities/ups';

export default class Api {
    private caller: AxiosInstance;

    constructor(cookies: any) {
        if (cookies) {
            this.caller = axios.create({
                baseURL: process.env.VUE_APP_API_ENDPOINT,
                headers: {
                    'X-OOM-AUTH': cookies.oom,
                },
                responseType: 'json',
            });
        } else {
            this.caller = axios.create({
                baseURL: process.env.VUE_APP_API_ENDPOINT,
                responseType: 'json',
            });
        }
    }

    public async login(code: string): Promise<AxiosResponse> {
        return this.caller.get('/users/login', { params: { code } });
    }

    public async loginInfo(): Promise<AxiosResponse> {
        return this.caller.get('/users/login/info');
    }

    public async createPost(newPost: NewPost): Promise<AxiosResponse> {
        return this.caller.post('/posts', newPost);
    }

    public async createComment(id: number, newComment: NewComment): Promise<AxiosResponse> {
        return this.caller.post(`/posts/${id}/comments`, newComment);
    }

    public async fetchQuestion(id: number): Promise<AxiosResponse> {
        return this.caller.get(`/questions/${id}`);
    }

    public async fetchAnswer(id: number): Promise<AxiosResponse> {
        return this.caller.get(`/answers/${id}`);
    }

    public async fetchAnswers(id: number, query: AnswersQuery): Promise<AxiosResponse> {
        return this.caller.get(`/questions/${id}/answers`);
    }

    public async fetchUserAnswers(id: number, query: AnswersQuery): Promise<AxiosResponse> {
        return this.caller.get(`/users/${id}/answers`);
    }

    public async fetchUserAnswersQuestions(
        id: number,
        query: AnswersQuery,
    ): Promise<AxiosResponse> {
        return this.caller.get(`/users/${id}/answers/questions`);
    }

    public async fetchComments(id: number): Promise<AxiosResponse> {
        return this.caller.get(`/posts/${id}/comments`);
    }

    public async fetchQuestions(query: QuestionsQuery): Promise<AxiosResponse> {
        return this.caller.get('/questions', { params: query });
    }

    public async fetchUserQuestions(id: number, query: QuestionsQuery): Promise<AxiosResponse> {
        return this.caller.get(`/users/${id}/questions`);
    }

    public async fetchUsers(query: UsersQuery): Promise<AxiosResponse> {
        return this.caller.get('/users', { params: query });
    }

    public async fetchUser(id: number): Promise<AxiosResponse> {
        return this.caller.get(`/users/${id}`);
    }

    public async fetchTag(id: number): Promise<AxiosResponse> {
        return this.caller.get(`/tags/${id}`);
    }

    public async fetchTags(query: TagsQuery): Promise<AxiosResponse> {
        return this.caller.get('/tags', { params: query });
    }

    public async fetchUserTags(id: number): Promise<AxiosResponse> {
        return this.caller.get(`/users/${id}/tags`);
    }

    public async fetchPostTags(id: number): Promise<AxiosResponse> {
        return this.caller.get(`/posts/${id}/tags`);
    }

    public async fetchTagQuestions(id: number): Promise<AxiosResponse> {
        return this.caller.get(`/questions/tag/${id}`);
    }

    public async searchTags(query: TagsSearchQuery): Promise<AxiosResponse> {
        return this.caller.get('/tags/search', { params: query });
    }

    public async createTag(newTag: NewTag): Promise<AxiosResponse> {
        return this.caller.post('tags', newTag);
    }

    public async createUserTag(tagId: number): Promise<AxiosResponse> {
        return this.caller.post(`/users/tags/${tagId}`);
    }

    public async createPostTag(postId: number, tagId: number): Promise<AxiosResponse> {
        return this.caller.post(`/posts/${postId}/tags/${tagId}`);
    }

    public async deleteUserTag(tagId: number): Promise<AxiosResponse> {
        return this.caller.delete(`/users/tags/${tagId}`);
    }

    public async deletePostTag(postId: number, tagId: number): Promise<AxiosResponse> {
        return this.caller.delete(`/posts/${postId}/tags/${tagId}`);
    }

    public async upsertUps(postId: number, newUp: NewUp): Promise<AxiosResponse> {
        return this.caller.patch(`/posts/${postId}/ups`, newUp);
    }

    public async isUp(postId: number): Promise<AxiosResponse> {
        return this.caller.get(`/posts/${postId}/ups`);
    }

    public async upsertViews(postId: number): Promise<AxiosResponse> {
        return this.caller.patch(`/posts/${postId}/views`);
    }

    public async updateBestAnswer(
        postId: number,
        updateBestAnswer: UpdateBestAnswer,
    ): Promise<AxiosResponse> {
        return this.caller.patch(`/posts/${postId}/best`, updateBestAnswer);
    }

    public async updateContent(
        postId: number,
        updateContent: UpdateContent,
    ): Promise<AxiosResponse> {
        return this.caller.patch(`/posts/${postId}`, updateContent);
    }
}
