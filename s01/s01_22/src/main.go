package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type User struct {
	password string
	balance  float64
}

type Bank struct {
	users map[string]User
}

func NewBank() *Bank {
	return &Bank{
		users: make(map[string]User),
	}
}

func (b *Bank) Register(username, password string) {
	if _, exists := b.users[username]; exists {
		fmt.Println("用户名已存在！")
	} else {
		b.users[username] = User{
			password: password,
			balance:  0.0,
		}
		fmt.Printf("用户 %s 注册成功！\n", username)
	}
}

func (b *Bank) Login(username, password string) bool {
	if user, exists := b.users[username]; exists {
		if user.password == password {
			fmt.Println("登录成功！")
			return true
		}
		fmt.Println("密码错误！")
	} else {
		fmt.Println("用户不存在！")
	}
	return false
}

func (b *Bank) Deposit(username string, amount float64) {
	user := b.users[username]
	user.balance += amount
	b.users[username] = user
	fmt.Printf("入金成功！当前余额：%.2f\n", user.balance)
}

func (b *Bank) Transfer(from, to string, amount float64) {
	if _, exists := b.users[to]; !exists {
		fmt.Println("接收用户不存在！")
		return
	}

	fromUser := b.users[from]
	if fromUser.balance < amount {
		fmt.Println("余额不足！")
		return
	}

	fromUser.balance -= amount
	toUser := b.users[to]
	toUser.balance += amount

	b.users[from] = fromUser
	b.users[to] = toUser

	fmt.Printf("转账成功！%s 向 %s 转账 %.2f 元\n", from, to, amount)
}

func (b *Bank) ShowAllUsers() {
	for username, user := range b.users {
		fmt.Printf("用户: %s, 余额: %.2f\n", username, user.balance)
	}
}

func main() {
	bank := NewBank()
	var loggedInUser string

	scanner := bufio.NewScanner(os.Stdin)

	for {
		if loggedInUser == "" {
			fmt.Println("请选择操作: 1. 注册 2. 登录 3. 查询所有用户 4. 退出")
			scanner.Scan()
			choice, _ := strconv.Atoi(scanner.Text())

			switch choice {
			case 1:
				username, password := getCredentials(scanner)
				bank.Register(username, password)
			case 2:
				username, password := getCredentials(scanner)
				if bank.Login(username, password) {
					loggedInUser = username
				}
			case 3:
				bank.ShowAllUsers()
			case 4:
				return
			default:
				fmt.Println("无效操作，请重新选择")
			}
		} else {
			fmt.Println("请选择操作: 1. 入金 2. 转账 3. 注销 4. 退出")
			scanner.Scan()
			choice, _ := strconv.Atoi(scanner.Text())

			switch choice {
			case 1:
				fmt.Print("请输入入金额: ")
				scanner.Scan()
				amount, _ := strconv.ParseFloat(scanner.Text(), 64)
				bank.Deposit(loggedInUser, amount)
			case 2:
				fmt.Print("请输入接收用户名: ")
				scanner.Scan()
				to := scanner.Text()
				fmt.Print("请输入转账金额: ")
				scanner.Scan()
				amount, _ := strconv.ParseFloat(scanner.Text(), 64)
				bank.Transfer(loggedInUser, to, amount)
			case 3:
				fmt.Printf("已注销 %s 的登录。\n", loggedInUser)
				loggedInUser = ""
			case 4:
				return
			default:
				fmt.Println("无效操作，请重新选择")
			}
		}
	}
}

func getCredentials(scanner *bufio.Scanner) (string, string) {
	fmt.Print("请输入用户名: ")
	scanner.Scan()
	username := scanner.Text()

	fmt.Print("请输入密码: ")
	scanner.Scan()
	password := scanner.Text()

	return username, password
}
