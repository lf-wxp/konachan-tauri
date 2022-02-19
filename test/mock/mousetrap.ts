export default {
  events: {},
  bindGlobal(key: string, handler: any) {
    //@ts-ignore
    this.events[key] = handler;
  },
  unbind(key: string) {
    //@ts-ignore
    this.events[key] = undefined;
  },
  trigger(key: string) {
    //@ts-ignore
    this.events?.[key]?.();
  }
};

