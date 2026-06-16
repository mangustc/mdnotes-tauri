import "solid-js";
import "@material/web/all";

declare module "solid-js" {
  namespace JSX {
    // Filter HTMLElementTagNameMap to only include 'md-' prefixed elements
    type MdElements = {
      [K in keyof HTMLElementTagNameMap as K extends `md-${string}`
        ? K
        : never]: HTMLElementTagNameMap[K];
    };

    // Map elements, omitting properties that conflict with standard JSX attributes
    type ElementProps<T> = {
      [K in keyof T]: Omit<
        Partial<T[K]>,
        "children" | "style" | "class" | "className"
      > &
        HTMLAttributes<T[K]> & {
          [P in keyof T[K] as `prop:${string & P}`]?: T[K][P];
        };
    };

    // Merge into Solid's IntrinsicElements
    interface IntrinsicElements extends ElementProps<MdElements> {}
  }
}
