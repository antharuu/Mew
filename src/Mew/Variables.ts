export module Variables {
    export let Data: Object = {}

    let Checkers = [
        { // Replace
            callback: (str: string) => {
                for (const [key, value] of Object.entries(Data)) {
                    const regex = new RegExp("{{([ ]+)?([" + key + "]+)([ ]+)?}}", "g")
                    str = str.replace(regex, value);
                }
                return str;
            }
        },
        { // Set or Change value
            callback: (str: string) => {
                const regex = /\$([\w]+)[ ]*?([=])(.*)/
                let m;
                while ((m = regex.exec(str)) !== null) {
                    if (m.index === regex.lastIndex) regex.lastIndex++;
                    let vName: string = m[1]
                    let vValue: string = m[3].trim()
                    if (vValue.charAt(0) === '"') vValue = vValue.substr(1, vValue.length - 2)
                    // @ts-ignore
                    Data[vName] = vValue;
                    console.log(Data)
                    str = ""
                }
                return str;
            }
        }
    ]

    export const check = (str: string) => {
        Checkers.forEach(checker => {
            str = checker.callback(str)
        })
        return str;
    };
}