import { Result, Option, ok, err } from "@/lib";
import { CreateResponse, HttpStatus } from "./common";
import axios, { AxiosInstance, AxiosResponse } from "axios";

const BASE_AUTH_ENDPOINT: string = "auth/v1";

export interface User {
  name: string;
  email: string;
  username: string;
  password: Option<string>;
}

export interface LoginResponse {
  user: User;
  token: string;
  refresh_token: string;
}

export interface RefreshResponse {
  token: string;
  refresh_token: string;
}

const parseCookie = (cookies: string[]): { [key: string]: string } => {
  const map: { [key: string]: string } = {};

  for (const cookie of cookies) {
    const separatedCookies = cookie.split(";");
    for (const kv of separatedCookies) {
      const [key, value] = kv.split("=");
      map[key] = value;
    }
  }

  return map;
};

export const login = async (
  username: string,
  password: string,
  client: AxiosInstance,
): Promise<Result<LoginResponse, string>> => {
  try {
    const response: AxiosResponse<User | CreateResponse> = await client.post(
      `/${BASE_AUTH_ENDPOINT}/login`,
      {
        username: username,
        password: password,
      },
    );

    if (response.status == HttpStatus.Ok) {
      const result: User = response.data as unknown as User;
      if (response.headers["set-cookie"] != null) {
        const cookies = response.headers["set-cookie"];

        const parsedCookies = parseCookie(cookies);

        const loginResponse: LoginResponse = {
          user: result,
          refresh_token: parsedCookies["refresh_token"],
          token: parsedCookies["token"],
        };
        return Promise.resolve(ok(loginResponse));
      }

      // if it reaches to this point means that there is something wrong with the backend
      return Promise.resolve(err("Could not login please try again"));
    }

    return Promise.resolve(
      err(`Unexpected status occured with data ${response.data}`),
    );
  } catch (error) {
    if (
      axios.isAxiosError(error) &&
      error.response !== undefined &&
      error.status == 400
    ) {
      const response = error.response.data as unknown as CreateResponse;
      return Promise.resolve(err(response.error_message));
    }

    return Promise.resolve(err((<Error>error).message));
  }
};

export const sign_up = async (
  user: User,
  client: AxiosInstance,
): Promise<Result<string, string>> => {
  try {
    const response: AxiosResponse<CreateResponse> = await client.post(
      `${BASE_AUTH_ENDPOINT}/sign_up`,
      user,
    );

    if (response.status == 200) {
      return Promise.resolve(ok("Sign up successful"));
    }

    return Promise.resolve(
      err(`Unexpected status occured with data ${response.data}`),
    );
  } catch (error) {
    if (
      axios.isAxiosError(error) &&
      error.response !== undefined &&
      error.status == 400
    ) {
      const response = error.response.data as unknown as CreateResponse;
      return Promise.resolve(err(response.error_message));
    }
    return Promise.resolve(err((<Error>error).message));
  }
};

export const refresh_token = async (
  client: AxiosInstance,
): Promise<Result<RefreshResponse, string>> => {
  try {
    const response: AxiosResponse<CreateResponse> = await client.get(
      `${BASE_AUTH_ENDPOINT}/refresh_token`,
    );

    if (response.headers["set-cookie"] != null) {
      const cookies = response.headers["set-cookie"];
      const parsedCookies = parseCookie(cookies);

      const result: RefreshResponse = {
        token: parsedCookies["token"],
        refresh_token: parsedCookies["refresh_token"],
      };

      return Promise.resolve(ok(result));
    }

    return Promise.resolve(err("Could not refresh the token"));
  } catch (error) {
    if (
      axios.isAxiosError(error) &&
      error.response !== undefined &&
      error.status == 400
    ) {
      const response = error.response.data as unknown as CreateResponse;
      return Promise.resolve(err(response.error_message));
    }
    return Promise.resolve(err((<Error>error).message));
  }
};
