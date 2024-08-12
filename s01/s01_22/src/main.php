<?php

class User {
    public $password;
    public $balance;

    public function __construct($password, $balance = 0) {
        $this->password = $password;
        $this->balance = $balance;
    }
}

class Bank {
    private $users = [];

    public function register($username, $password) {
        if (isset($this->users[$username])) {
            echo "用户名已存在！\n";
        } else {
            $this->users[$username] = new User($password);
            echo "用户 {$username} 注册成功！\n";
        }
    }

    public function login($username, $password) {
        if (isset($this->users[$username])) {
            if ($this->users[$username]->password === $password) {
                echo "登录成功！\n";
                return true;
            } else {
                echo "密码错误！\n";
            }
        } else {
            echo "用户不存在！\n";
        }
        return false;
    }

    public function deposit($username, $amount) {
        if (isset($this->users[$username])) {
            $this->users[$username]->balance += $amount;
            echo "入金成功！当前余额：{$this->users[$username]->balance}\n";
        } else {
            echo "用户不存在！\n";
        }
    }

    public function transfer($from, $to, $amount) {
        if (!isset($this->users[$to])) {
            echo "接收用户不存在！\n";
            return;
        }

        if (isset($this->users[$from])) {
            if ($this->users[$from]->balance < $amount) {
                echo "余额不足！\n";
            } else {
                $this->users[$from]->balance -= $amount;
                $this->users[$to]->balance += $amount;
                echo "转账成功！{$from} 向 {$to} 转账 {$amount} 元\n";
            }
        } else {
            echo "发送用户不存在！\n";
        }
    }

    public function showAllUsers() {
        foreach ($this->users as $username => $user) {
            echo "用户: {$username}, 余额: {$user->balance}\n";
        }
    }
}

function getInput($prompt) {
    echo $prompt;
    return trim(fgets(STDIN));
}

$bank = new Bank();
$loggedInUser = null;

while (true) {
    if ($loggedInUser === null) {
        echo "请选择操作: 1. 注册 2. 登录 3. 查询所有用户 4. 退出\n";
        $choice = getInput("");

        switch ($choice) {
            case '1':
                $username = getInput("请输入用户名: ");
                $password = getInput("请输入密码: ");
                $bank->register($username, $password);
                break;
            case '2':
                $username = getInput("请输入用户名: ");
                $password = getInput("请输入密码: ");
                if ($bank->login($username, $password)) {
                    $loggedInUser = $username;
                }
                break;
            case '3':
                $bank->showAllUsers();
                break;
            case '4':
                exit("退出系统\n");
            default:
                echo "无效操作，请重新选择\n";
        }
    } else {
        echo "请选择操作: 1. 入金 2. 转账 3. 注销 4. 退出\n";
        $choice = getInput("");

        switch ($choice) {
            case '1':
                $amount = getInput("请输入入金额: ");
                $bank->deposit($loggedInUser, (float)$amount);
                break;
            case '2':
                $to = getInput("请输入接收用户名: ");
                $amount = getInput("请输入转账金额: ");
                $bank->transfer($loggedInUser, $to, (float)$amount);
                break;
            case '3':
                echo "已注销 {$loggedInUser} 的登录。\n";
                $loggedInUser = null;
                break;
            case '4':
                exit("退出系统\n");
            default:
                echo "无效操作，请重新选择\n";
        }
    }
}

