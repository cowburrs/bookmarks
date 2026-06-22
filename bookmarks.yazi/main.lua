local M = {}

local state = ya.sync(function(st)
	return {
		cwd = tostring(cx.active.current.cwd),
		empty = st.empty,
	}
end)

function M:setup(opts)
	opts = opts or {}
end

function M:entry()
	local st = state()

	local permit = ui.hide()
	local target, err = M.run_with(st.cwd)
	permit:drop()

	local output, err = Command("bm"):arg({ "go", "-r", target }):stdout(Command.PIPED):output()
	if not output then
		return nil, Err("Failed to run `bm go`, error: %s", err)
	end
	local path = output.stdout:gsub("\n$", "")

	if not target then
		ya.notify({ title = "bookmarks", content = tostring(err), timeout = 5, level = "error" })
	elseif target ~= "" then
		ya.emit("cd", { path, raw = true })
	end
end

function M.options()
	-- https://github.com/ajeetdsouza/zoxide/blob/main/src/cmd/query.rs#L92
	local default = {
		-- Search mode
		"--exact",
		-- Search result
		"--no-sort",
		-- Interface
		"--bind=ctrl-z:ignore,btab:up,tab:down",
		"--cycle",
		"--keep-right",
		-- Layout
		"--layout=reverse",
		"--height=100%",
		"--border",
		"--scrollbar=▌",
		"--info=inline",
		-- Display
		"--tabstop=1",
		-- Scripting
		"--exit-0",
	}

	if ya.target_family() == "unix" then
		default[#default + 1] = "--preview-window=down,30%,sharp"
		if ya.target_os() == "linux" then
			default[#default + 1] = [[--preview='\command -p bm list {1}']]
		end
	end

	return (os.getenv("FZF_DEFAULT_OPTS") or "")
		.. " "
		.. table.concat(default, " ")
		.. " "
		.. (os.getenv("YAZI_ZOXIDE_OPTS") or "")
end

---@param cwd string
---@return boolean
function M.is_empty(cwd)
	return false -- TODO: Make a cheap thing later.
end

---@param cwd string
---@return string?, Error?
function M.run_with(cwd)
	local child, err = Command("sh")
		:arg({ "-c", "bm list -s | fzf " .. M.options() })
		:env("SHELL", "sh")
		:env("CLICOLOR", 1)
		:env("CLICOLOR_FORCE", 1)
		:stdin(Command.INHERIT)
		:stdout(Command.PIPED)
		:stderr(Command.PIPED)
		:spawn()
	if not child then
		return nil, Err("Failed to start `fzf`, error: %s", err)
	end

	local output, oerr = child:wait_with_output()
	if not output then
		return nil, Err("Cannot read `fzf` output, error: %s", oerr)
	elseif not output.status.success and output.status.code ~= 130 then
		return nil, Err("`fzf` exited with code %s: %s", output.status.code, output.stderr)
	end

	local selected = output.stdout:gsub("\n$", "")
	local path = selected:match("\t(.+)$") or selected
	return path, nil
end

return M
