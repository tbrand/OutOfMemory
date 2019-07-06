import queryString from 'query-string';

export type OomCookie = {
    token: string,
    userId: number,
}

export default function parseCookie(cookie: any): OomCookie {
    const parsedCookie = queryString.parse(cookie.oom);

    return {
        token: parsedCookie.token as string,
        userId: Number(parsedCookie.user_id),
    };
}
