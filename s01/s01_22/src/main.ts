class User {
    password: string;
    balance: number;

    constructor(password: string, balance: number = 0) {
        this.password = password;
        this.balance = balance;
    }
}

class Bank {
    private users: Map<string, User> = new Map();

    register(username: string, password: string): void {
        if (this.users.has(username)) {
            console.log("用户名已存在！");
        } else {
            this.users.set(username, new User(password));
            console.log(`用户 ${username} 注册成功！`);
        }
    }

    login(username: string, password: string): boolean {
        const user = this.users.get(username);
        if (user) {
            if (user.password === password) {
                console.log("登录成功！");
                return true;
            } else {
                console.log("密码错误！");
            }
        } else {
            console.log("用户不存在！");
        }
        return false;
    }

    deposit(username: string, amount: number): void {
        const user = this.users.get(username);
        if (user) {
            user.balance += amount;
            console.log(`入金成功！当前余额：${user.balance}`);
        } else {
            console.log("用户不存在！");
        }
    }

    transfer(from: string, to: string, amount: number): void {
        const fromUser = this.users.get(from);
        const toUser = this.users.get(to);

        if (!toUser) {
            console.log("接收用户不存在！");
            return;
        }

        if (fromUser) {
            if (fromUser.balance < amount) {
                console.log("余额不足！");
            } else {
                fromUser.balance -= amount;
                toUser.balance += amount;
                console.log(`转账成功！${from} 向 ${to} 转账 ${amount} 元`);
            }
        } else {
            console.log("发送用户不存在！");
        }
    }

    showAllUsers(): void {
        this.users.forEach((user, username) => {
            console.log(`用户: ${username}, 余额: ${user.balance}`);
        });
    }
}

function getInput(prompt: string): Promise<string> {
    return new Promise((resolve) => {
        process.stdout.write(prompt);
        process.stdin.once('data', data => resolve(data.toString().trim()));
    });
}

async function main() {
    const bank = new Bank();
    let loggedInUser: string | null = null;

    while (true) {
        if (!loggedInUser) {
            console.log("请选择操作: 1. 注册 2. 登录 3. 查询所有用户 4. 退出");
            const choice = await getInput("");

            switch (choice) {
                case '1':
                    const username = await getInput("请输入用户名: ");
                    const password = await getInput("请输入密码: ");
                    bank.register(username, password);
                    break;
                case '2':
                    const loginUsername = await getInput("请输入用户名: ");
                    const loginPassword = await getInput("请输入密码: ");
                    if (bank.login(loginUsername, loginPassword)) {
                        loggedInUser = loginUsername;
                    }
                    break;
                case '3':
                    bank.showAllUsers();
                    break;
                case '4':
                    process.exit();
                default:
                    console.log("无效操作，请重新选择");
            }
        } else {
            console.log("请选择操作: 1. 入金 2. 转账 3. 注销 4. 退出");
            const choice = await getInput("");

            switch (choice) {
                case '1':
                    const amount = parseFloat(await getInput("请输入入金额: "));
                    bank.deposit(loggedInUser, amount);
                    break;
                case '2':
                    const toUsername = await getInput("请输入接收用户名: ");
                    const transferAmount = parseFloat(await getInput("请输入转账金额: "));
                    bank.transfer(loggedInUser, toUsername, transferAmount);
                    break;
                case '3':
                    console.log(`已注销 ${loggedInUser} 的登录。`);
                    loggedInUser = null;
                    break;
                case '4':
                    process.exit();
                default:
                    console.log("无效操作，请重新选择");
            }
        }
    }
}

main();
