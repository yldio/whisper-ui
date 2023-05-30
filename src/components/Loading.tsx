export default () => {
  const common = 'h-full w-2 scale-y-40 rounded-lg bg-aquamarine-500';
  return (
    <div class="mx-auto my-5 flex h-16 space-x-2">
      <div class={`${common} animate-quiet`}></div>
      <div class={`${common} animate-normal`}></div>
      <div class={`${common} animate-quiet`}></div>
      <div class={`${common} animate-loud`}></div>
      <div class={`${common} animate-quiet`}></div>
    </div>
  );
};
