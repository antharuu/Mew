import {BlockElement} from "../Logic/BlockElement";

export interface IAction {
    (newElement: BlockElement, oldElement?: BlockElement): BlockElement;
}