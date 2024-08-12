defmodule Bank do
  defstruct users: %{}

  def start() do
    %Bank{}
  end

  def register(bank, username, password) do
    if Map.has_key?(bank.users, username) do
      IO.puts("用户名已存在！")
      bank
    else
      new_user = %{password: password, balance: 0.0}
      updated_users = Map.put(bank.users, username, new_user)
      IO.puts("用户 #{username} 注册成功！")
      %Bank{bank | users: updated_users}
    end
  end

  def login(bank, username, password) do
    case Map.get(bank.users, username) do
      nil ->
        IO.puts("用户不存在！")
        {:error, bank}

      user ->
        if user.password == password do
          IO.puts("登录成功！")
          {:ok, bank, username}
        else
          IO.puts("密码错误！")
          {:error, bank}
        end
    end
  end

  def deposit(bank, username, amount) do
    user = Map.get(bank.users, username)
    updated_user = %{user | balance: user.balance + amount}
    updated_users = Map.put(bank.users, username, updated_user)
    IO.puts("入金成功！当前余额：#{updated_user.balance}")
    %Bank{bank | users: updated_users}
  end

  def transfer(bank, from, to, amount) do
    if Map.has_key?(bank.users, to) do
      from_user = Map.get(bank.users, from)
      to_user = Map.get(bank.users, to)

      if from_user.balance < amount do
        IO.puts("余额不足！")
        bank
      else
        updated_from_user = %{from_user | balance: from_user.balance - amount}
        updated_to_user = %{to_user | balance: to_user.balance + amount}

        updated_users =
          bank.users
          |> Map.put(from, updated_from_user)
          |> Map.put(to, updated_to_user)

        IO.puts("转账成功！#{from} 向 #{to} 转账 #{amount} 元")
        %Bank{bank | users: updated_users}
      end
    else
      IO.puts("接收用户不存在！")
      bank
    end
  end

  def show_all_users(bank) do
    Enum.each(bank.users, fn {username, user} ->
      IO.puts("用户: #{username}, 余额: #{user.balance}")
    end)
  end
end

defmodule Main do
  def start() do
    bank = Bank.start()
    loop(bank, nil)
  end

  defp loop(bank, nil) do
    IO.puts("请选择操作: 1. 注册 2. 登录 3. 查询所有用户 4. 退出")
    choice = IO.gets("") |> String.trim() |> String.to_integer()

    case choice do
      1 ->
        {username, password} = get_credentials()
        new_bank = Bank.register(bank, username, password)
        loop(new_bank, nil)

      2 ->
        {username, password} = get_credentials()

        case Bank.login(bank, username, password) do
          {:ok, new_bank, username} ->
            loop(new_bank, username)

          {:error, new_bank} ->
            loop(new_bank, nil)
        end

      3 ->
        Bank.show_all_users(bank)
        loop(bank, nil)

      4 ->
        IO.puts("退出系统")
        :ok

      _ ->
        IO.puts("无效操作，请重新选择")
        loop(bank, nil)
    end
  end

  defp loop(bank, logged_in_user) do
    IO.puts("请选择操作: 1. 入金 2. 转账 3. 注销 4. 退出")
    choice = IO.gets("") |> String.trim() |> String.to_integer()

    case choice do
      1 ->
        IO.write("请输入入金额: ")
        amount = IO.gets("") |> String.trim() |> String.to_float()
        new_bank = Bank.deposit(bank, logged_in_user, amount)
        loop(new_bank, logged_in_user)

      2 ->
        IO.write("请输入接收用户名: ")
        to = IO.gets("") |> String.trim()
        IO.write("请输入转账金额: ")
        amount = IO.gets("") |> String.trim() |> String.to_float()
        new_bank = Bank.transfer(bank, logged_in_user, to, amount)
        loop(new_bank, logged_in_user)

      3 ->
        IO.puts("已注销 #{logged_in_user} 的登录。")
        loop(bank, nil)

      4 ->
        IO.puts("退出系统")
        :ok

      _ ->
        IO.puts("无效操作，请重新选择")
        loop(bank, logged_in_user)
    end
  end

  defp get_credentials() do
    IO.write("请输入用户名: ")
    username = IO.gets("") |> String.trim()
    IO.write("请输入密码: ")
    password = IO.gets("") |> String.trim()
    {username, password}
  end
end

Main.start()
