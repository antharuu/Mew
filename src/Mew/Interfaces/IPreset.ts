import {IElement} from "./IElement";
import {IAction} from "./IAction";

export interface IPreset {
    tag: string,
    element: IElement,
    action: IAction
}