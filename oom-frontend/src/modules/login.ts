import Api from './api';

export default async function login() {
    const api = new Api(null);
    const res = await api.loginInfo();

    window.location.href = `${res.data.oauth_authorize}?client_id=${res.data.client_id}`;
}
