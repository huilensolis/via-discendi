"use client";

import { login, User } from "@/api";
import { isOk, none, some, Option } from "@/lib";
import axios from "axios";
import { createContext, useState } from "react";

export interface AuthProviderData {
  user: Option<User>;
  setUser: (username: string, password: string) => void;
}

// TODO: think about how to instantiate the axios client better
const axiosClient = axios.create();
export const AuthContext = createContext<Option<AuthProviderData>>(none());

export function Providers({ children }: any) {
  const [user, setUser] = useState<Option<User>>(none());

  const loginWrapper = async (
    username: string,
    password: string,
  ): Promise<Option<string>> => {
    const response = await login(username, password, axiosClient);
    if (isOk(response)) {
      // TODO: think about putting the token on the axios client
      const { user } = response.value;
      setUser(some(user));
      return none();
    } else {
      return some(response.value);
    }
  };
  return (
    <AuthContext.Provider value={some({ user: user, setUser: loginWrapper })}>
      {children}
    </AuthContext.Provider>
  );
}
