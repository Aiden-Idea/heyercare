// useCache.ts
import { ref } from "vue";

export const kEEP_ALIVE_MAP = ref(new Map([]));

export default () => {
  const setKeepAlive = (key: string, value: number) => {
    kEEP_ALIVE_MAP.value.set(key, value);
  };

  return {
    kEEP_ALIVE_MAP,
    setKeepAlive
  };
};