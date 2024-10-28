"use client";

import { useContext } from "react";
import { AuthContext } from "../providers";
import { isSome, none } from "@/lib";

export default function Page() {
  const data = useContext(AuthContext);
  const user = isSome(data) ? data.value.user : none();
  const userName = isSome(user) ? user.value.name : "Unknown person";

  return <div>{`Hello ${userName}`}</div>;
}
