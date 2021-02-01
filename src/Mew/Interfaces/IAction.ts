export interface IAction {
    (newElement: Element, oldElement?: Element): Element;
}