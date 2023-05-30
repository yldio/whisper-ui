import { children, Component, JSX } from 'solid-js';

const Layout: Component<{ hovering: boolean; children: JSX.Element }> = (
  props,
) => {
  const c = children(() => props.children);
  return (
    <div class="relative h-screen w-screen select-none overflow-hidden bg-blackrock-950 p-2 text-center text-blackrock-50">
      <div
        class={`h-full w-full rounded-lg border-2 border-dashed border-blackrock-500 align-middle ${
          props.hovering ? 'blur-[1px]' : 'brightness-125'
        }`}
      >
        {c()}
      </div>
    </div>
  );
};

export default Layout;
