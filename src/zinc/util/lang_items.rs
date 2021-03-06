// Zinc, the bare metal stack for rust.
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::fmt::Arguments;

#[cfg(not(test))]
#[lang="stack_exhausted"]
#[no_split_stack]
extern fn stack_exhausted() {}

#[cfg(not(test))]
#[lang="eh_personality"]
#[no_split_stack]
extern fn eh_personality() {}

#[cfg(not(test))]
#[lang="begin_unwind"]
#[no_split_stack]
extern fn begin_unwind() {}

#[cfg(not(test))]
#[lang="fail_fmt"]
#[no_split_stack]
pub fn fail_fmt(fmt: &Arguments, file_line: &(&'static str, uint)) -> ! {
  loop { }
}
