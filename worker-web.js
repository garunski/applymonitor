// Worker script to serve static files from Workers Sites
export default {
  async fetch(request, env) {
    return env.ASSETS.fetch(request);
  }
};
